use std::fs;
use std::io;

use crate::dir_helpers::DirInfo;
use crate::wipe_params::WipeParams;
use crate::writer::Writer;

#[derive(Debug)]
pub struct Wipe<'a, W>
where
    W: io::Write,
{
    writer: Writer<'a, W>,
    params: &'a WipeParams,
}

impl<'a, W> Wipe<'a, W>
where
    W: io::Write,
{
    pub fn new(stdout: &'a mut W, params: &'a WipeParams) -> Self {
        let writer = Writer::new(stdout);

        Self { writer, params }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let writer = &mut self.writer;
        let params = self.params;

        writer.write_header(params)?;

        let paths_to_delete = DirInfo::get_paths_to_delete(&params.path, &params.language)?;
        let paths_to_delete = paths_to_delete
            .iter()
            .filter_map(|p| p.as_ref().ok())
            .collect::<Vec<_>>();

        let previous_info = if paths_to_delete.is_empty() {
            None
        } else {
            writer.write_content_header()?;
            Some(DirInfo::dir_size(&params.path)?)
        };

        let mut wipe_info = DirInfo::new(paths_to_delete.len(), 0, 0);
        let mut ignore_info = DirInfo::new(0, 0, 0);

        let paths_ignored = params
            .ignores
            .iter()
            .map(|p| p.display().to_string().to_lowercase())
            .collect::<Vec<_>>();

        for path in paths_to_delete {
            let dir_info = DirInfo::dir_size(path);

            let ignored = paths_ignored
                .iter()
                .any(|p| path.to_lowercase().starts_with(p));

            let error = if !ignored && params.wipe {
                fs::remove_dir_all(path).err()
            } else {
                None
            };

            if let Ok(dir_info) = dir_info {
                if ignored {
                    ignore_info.dir_count += 1;
                    ignore_info.file_count += dir_info.file_count;
                    ignore_info.size += dir_info.size;
                } else {
                    wipe_info.file_count += dir_info.file_count;
                    wipe_info.size += dir_info.size;
                }
            }

            writer.write_content_line(path, dir_info, ignored, error)?;
        }

        writer.write_summary(params, &wipe_info, &ignore_info, &previous_info)?;
        writer.write_footer(params, &wipe_info)?;

        Ok(())
    }
}
