use futures::StreamExt;
use is_odd::IsOdd;
use prime_checker::is_prime;
use stream_broadcast::StreamBroadcastUnlimitedExt;
use tokio::join;

#[tokio::main]
async fn main() {
    let broadcast = futures::stream::iter(1..=50).fuse().broadcast_unlimited();

    let odd_stream = broadcast.clone().map(|(_index, value)| value).filter(|v| {
        let is_odd = v.is_odd();
        async move { is_odd }
    });
    let even_stream = broadcast.clone().map(|(_index, value)| value).filter(|v| {
        let is_even = !v.is_odd();
        async move { is_even }
    });
    let prime_stream = broadcast.clone().map(|(_index, value)| value).filter(|v| {
        let (is_prime, _factors) = is_prime(v.to_owned() as u64);
        async move { is_prime }
    });

    join!(
        async {
            println!("Odd numbers: {:?}", odd_stream.collect::<Vec<_>>().await);
        },
        async {
            println!("Even numbers: {:?}", even_stream.collect::<Vec<_>>().await);
        },
        async {
            println!(
                "Prime numbers: {:?}",
                prime_stream.collect::<Vec<_>>().await
            );
        }
    );
}
