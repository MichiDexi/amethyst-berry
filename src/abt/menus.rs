#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
	Main,

	MapSelect,
	MapView,
	LobbySelect,
	SavefileSelect,
	UserSelect,

	ModBrowser,
	ModManage,

	CelesteExecutable,
	
	Tier,
	Achievements,
	Wiki,

	Out
}
