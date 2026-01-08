#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
	Main,

	MapSelect,
	MapView,
	PracticeView,
	LobbySelect,
	SavefileSelect,
	UserSelect,

	ModBrowser,
	ModManage,

	CelesteExecutable,
	Launch,

	Speedruns,
	Tier,
	Challenges,
	Wiki,

	Out
}
