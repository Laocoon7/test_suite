use crate::prelude::*;

pub trait SaveLoad: AsRef<Path> {
    fn get_reader(&self) -> Result<BufReader<File>> {
        let file = File::open(self)?;
        Ok(BufReader::new(file))
    }

    fn get_writer(&self) -> Result<BufWriter<File>> {
        let file = File::create(self)?;
        Ok(BufWriter::new(file))
    }

    fn load_file(&self) -> Result<String> {
        let mut reader = self.get_reader()?;
        let mut contents = String::new();
        let _bytes_read = reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn load_bytes(&self) -> Result<Vec<u8>> {
        let data = std::fs::read(self)?;
        Ok(data)
    }

    fn save_file(&self, contents: &str) -> Result<()> {
        if let Some(dir) = self.as_ref().parent() {
            create_dir_all(dir)?;
        }
        let mut writer = self.get_writer()?;
        writer.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn from_toml<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = self.load_file()?;
        let value = toml::from_str::<T>(&contents)?;
        Ok(value)
    }

    fn from_ron<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = self.load_file()?;
        let value = ron::from_str::<T>(&contents)?;
        Ok(value)
    }

    fn to_toml<T: Serialize>(&self, value: &T) -> Result<()> {
        let contents = toml::to_string_pretty(value)?;
        self.save_file(&contents)?;
        Ok(())
    }

    fn to_ron<T: Serialize>(&self, value: &T) -> Result<()> {
        let contents = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())?;
        self.save_file(&contents)?;
        Ok(())
    }

    fn path_string(&self) -> String {
        if let Ok(s) = AsRef::<Path>::as_ref(self).canonicalize() {
            if let Some(s) = s.to_str() {
                s.to_string()
            } else {
                "UNABLE/TO/CONVERT/PATH/TO/STRING".to_string()
            }
        } else {
            "INVALID/PATH".to_string()
        }
    }
}

impl<T: AsRef<Path>> SaveLoad for T {}
