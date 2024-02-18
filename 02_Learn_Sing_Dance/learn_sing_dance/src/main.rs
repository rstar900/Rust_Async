
use futures::executor::block_on;

struct Song {
    name: String,
    author: String,
    lyrics: String
}

async fn learn_song() -> Song {
    // Suppose learning the song could be an asynchronous sleep, 
    // then dance() could be run even before this task is completed
    println!("Learning the song..."); 

    Song {
        name: String::from("Vacancy"),
        author: String::from("Switch Disco"),
        lyrics: String::from("For you, baby\nMy heart got a vacancy\nI need you to take the key\nI got room for you...")
    }
}

async fn sing_song(song: Song) {
    println!("Singing 🎶🎵 {} by {} and it goes like this:\n{}", song.name, song.author, song.lyrics);
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {
    println!("Dancing to the rythm 🙆‍♂️💁‍♂️🙋‍♂️🤷‍♂️");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {

    block_on(async_main());
}
