//! Bossa Nova music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Bossa Nova artist
pub fn artist() -> String {
    fetch_locale("bossa_nova.artists", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ARTISTS).to_string())
}

/// Generate a random Bossa Nova song
pub fn song() -> String {
    fetch_locale("bossa_nova.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

// Fallback data
const FALLBACK_ARTISTS: &[&str] = &[
    "João Gilberto",
    "Antônio Carlos Jobim",
    "Vinícius de Moraes",
    "Astrud Gilberto",
    "Stan Getz",
    "Elis Regina",
    "Nara Leão",
    "Gal Costa",
    "Caetano Veloso",
    "Gilberto Gil",
    "Maria Bethânia",
    "Chico Buarque",
    "Milton Nascimento",
    "Edu Lobo",
    "Baden Powell",
    "Luiz Bonfá",
];

const FALLBACK_SONGS: &[&str] = &[
    "The Girl from Ipanema",
    "Garota de Ipanema",
    "Desafinado",
    "Corcovado",
    "Águas de Março",
    "Wave",
    "Meditation",
    "One Note Samba",
    "Dindi",
    "Inútil Paisagem",
    "Chega de Saudade",
    "Corcovado (Quiet Nights of Quiet Stars)",
    "Triste",
    "Once I Loved",
    "How Insensitive",
    "A Felicidade",
    "Só Tinha de Ser Com Você",
    "O Pato",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist() {
        assert!(!artist().is_empty());
    }

    #[test]
    fn test_song() {
        assert!(!song().is_empty());
    }
}
