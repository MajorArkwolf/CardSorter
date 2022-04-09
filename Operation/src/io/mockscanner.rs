use super::scanner::Scanner;

struct MockScanner {}

impl Scanner for MockScanner {
    fn setup(&mut self) -> color_eyre::Result<()> {
        todo!()
    }

    fn shutdown(&mut self) -> color_eyre::Result<()> {
        todo!()
    }

    fn read(&mut self) {
        todo!()
    }

    fn status(&mut self) -> color_eyre::Result<()> {
        todo!()
    }
}