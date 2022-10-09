use tokio::io::AsyncRead;

pub struct ZipFileReader<R> where R: AsyncRead + Unpin {
    pub(crate) inner: R
}

pub struct ZipEntryReader<R> where R: AsyncRead + Unpin {
    pub(crate) inner: R,
}