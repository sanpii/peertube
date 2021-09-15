use clap::Clap;

#[derive(Clap)]
struct Opt {
    #[clap(long)]
    instance: String,
    #[clap(long)]
    username: String,
    #[clap(long)]
    password: String,
    #[clap(long)]
    channel: String,
    url: String,
}

#[tokio::main]
async fn main() -> peertube::Result {
    env_logger::init();

    let opt = Opt::parse();
    let peertube = peertube::Api::new(&opt.instance);
    let auth = peertube.auth(&opt.username, &opt.password).await?;
    let channel = peertube.channels.get(&opt.channel).await?;

    let import = peertube::param::Import {
        video: peertube::param::NewVideo {
            channel_id: channel.id,
            privacy: Some(peertube::param::Privacy::Public),

            ..Default::default()
        },
        target_url: Some(opt.url),

        ..Default::default()
    };

    peertube.videos.import(&auth, &import).await?;

    Ok(())
}
