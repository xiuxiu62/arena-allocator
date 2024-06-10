use arena_allocator::{Arena, ArenaResult};

fn main() -> ArenaResult<()> {
    let mut arena = Arena::<512, 8>::new()?;
    let first = arena.allocate::<[u8; 256]>()?;
    let second = arena.allocate::<[u8; 256]>()?;

    (0..=255).for_each(|i| first[i] = i as u8);
    println!("{}", arena.dump()?);

    (0..=255).rev().for_each(|i| second[255 - i] = i as u8);
    println!("{}", arena.dump()?);

    (0..=255).for_each(|i| first[i] = 0);
    println!("{}", arena.dump()?);

    (0..=255).rev().for_each(|i| second[255 - i] = 0);
    println!("{}", arena.dump()?);

    Ok(())
}
