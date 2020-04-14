#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().map(|score| *score)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().map(|score| *score).max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vec = self.scores().to_vec();
        vec.sort_by(|a, b| b.cmp(a));

        vec.iter().take(3).map(|score| *score).collect()
    }
}