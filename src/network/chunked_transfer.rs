use crate::network::Session;
use crate::{GameError, GameErrorKind, ProgressState};
use std::sync::Arc;

pub struct ChunkedTransfer;

impl ChunkedTransfer {
    pub(crate) const AVATAR_CHUNK_MAXIMUM_LENGTH: u16 = 16384;
    pub(crate) const ACCOUNT_CHUNK_MAXIMUM_COUNT: u16 = 8;
    pub(crate) const EDITABLE_UNIT_CHUNK_MAXIMUM_COUNT: u16 = 16;

    pub async fn download_bytes(
        request_writer: impl AsyncFn(i32, u16) -> Result<Session, GameError>,
        progress_state: Option<Arc<ProgressState>>,
        description: impl Into<String>,
    ) -> Result<Vec<u8>, GameError> {
        let description = description.into();
        let mut offset = 0_usize;
        let mut data = None::<Vec<u8>>;

        if let Some(progress) = &progress_state {
            progress.reset();
        }

        loop {
            let response = request_writer(offset as i32, Self::AVATAR_CHUNK_MAXIMUM_LENGTH)
                .await?
                .response()
                .await?;

            let (total_length, returned_offset, chunk_length, mut packet) =
                GameError::check(response, |mut packet| {
                    let (total_length, returned_offset, chunk_length) = packet.read(|reader| {
                        (
                            reader.read_int32(),
                            reader.read_int32(),
                            reader.read_uint16(),
                        )
                    });
                    Ok((total_length, returned_offset, chunk_length, packet))
                })?;

            if returned_offset as usize != offset {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!(
                        "Unexpected {description} chunk offset returned_offset={returned_offset}, expected={offset}"
                    )),
                }
                .into());
            }

            let data_slice = match &mut data {
                Some(items) => items,
                None => {
                    if total_length < 0 {
                        return Err(GameErrorKind::InvalidData {
                            message: Some(format!("Negative {description}")),
                        }
                        .into());
                    } else {
                        data.insert(Vec::with_capacity(total_length as usize))
                    }
                }
            };

            if total_length as usize != data_slice.capacity() {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!(
                        "Unexpected {description} total length {total_length}, expected={}",
                        data_slice.capacity()
                    )),
                }
                .into());
            }

            if (offset + chunk_length as usize) > data_slice.capacity() {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!(
                        "The received {description} chunk overruns the announced total length."
                    )),
                }
                .into());
            }

            packet.read(|reader| {
                reader.fill_bytes(&mut data_slice[offset..][..chunk_length as usize]);
            });

            offset += chunk_length as usize;
            if let Some(progress) = &progress_state {
                progress.report(offset as _, data_slice.len() as _);
            }

            if offset == data_slice.len() {
                return Ok(data.expect("We just worked on it, it cannot fail."));
            }

            if chunk_length == 0 {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!("The server returned an empty {description} chunk before the transfer finished."))
                }.into());
            }
        }
    }

    /*
    pub async fn download_items<T>(
        self,
        max_count: u16,
        description: impl Into<String>,
        item_reader: impl Fn(&mut dyn PacketReader) -> Result<T, GameError>,
    ) -> Result<Vec<T>, GameError> {
        let description = description.into();
        let mut offset = 0_usize;
        let mut items = None::<Vec<T>>;

        if let Some(progress) = &self.progress {
            progress.reset();
        }

        for _ in 0..max_count {
            let (total_count, returned_offset, chunk_count, mut packet) =
                GameError::check(self.session.next().await?, |mut packet| {
                    let (total_count, returned_offset, chunk_count) = packet.read(|reader| {
                        (
                            reader.read_int32(),
                            reader.read_int32(),
                            reader.read_uint16(),
                        )
                    });
                    Ok((total_count, returned_offset, chunk_count, packet))
                })?;

            if returned_offset as usize != offset {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!(
                        "Unexpected {description} chunk offset returned_offset={returned_offset}, expected={offset}"
                    )),
                }
                    .into());
            }

            let items_vec = match &mut items {
                Some(items) => items,
                None => {
                    if total_count < 0 {
                        return Err(GameErrorKind::InvalidData {
                            message: Some(format!("Negative {description}")),
                        }
                        .into());
                    } else if total_count == 0 {
                        return Ok(Vec::new());
                    } else {
                        items.insert(Vec::with_capacity(total_count as usize))
                    }
                }
            };

            if total_count as usize != items_vec.capacity() {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!(
                        "Unexpected {description} total count {total_count}, expected={}",
                        items_vec.capacity()
                    )),
                }
                .into());
            }

            if (offset + chunk_count as usize) > items_vec.capacity() {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!(
                        "The received {description} chunk overruns the announced total count."
                    )),
                }
                .into());
            }

            packet.read(|reader| {
                for i in 0..chunk_count {
                    let item = item_reader(reader)?;
                    items_vec.insert(offset + i as usize, item);
                }
                Ok::<(), GameError>(())
            })?;

            offset += chunk_count as usize;
            if let Some(progress) = &self.progress {
                progress.report(offset as _, items_vec.len() as _);
            }

            if offset == items_vec.len() {
                return Ok(items.expect("We just worked on it, it cannot fail."));
            }

            if chunk_count == 0 {
                return Err(GameErrorKind::InvalidData {
                    message: Some(format!("The server returned an empty {description} chunk before the transfer finished."))
                }.into());
            }
        }

        Err(GameErrorKind::InvalidData {
            message: Some(format!("Reached maximum item count of {max_count}")),
        }
        .into())
    }
     */
}
