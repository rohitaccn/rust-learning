use tokio;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let event_stream = tokio::fs::read_dir("/path/to/events")
            .await
            .map_err(|e| e.into())
            .and_then(|entries| {
                futures::stream::iter(entries)
                    .map(|entry| entry.map_err(|e| e.into()))
                    .filter_map(|entry| async {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .await
            });
        event_stream
            .and_then(|events| {
                let event_futures = events
                    .into_iter()
                    .map(|event| {
                        tokio::fs::read(event).map(|data| (event, data))
                    })
                    .collect::<Vec<_>>();
                future::join_all(event_futures)
            })
            .and_then(|event_data| {
                event_data
                    .into_iter()
                    .for_each(|(event, data)| {
                        println!("Event {:?} has data {:?}", event, data);
                    });
                future::ready(())
            })
            .await;
        Ok(())
    })
}
