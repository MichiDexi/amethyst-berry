#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
	Main,

	MapSelect,
	MapView,
	MapEdit,
	MapCreate,
	MapDelete,
	MapMove,
	MapPractice,

	LobbySelect,
	LobbyView,
	LobbyEdit,
	LobbyCreate,
	LobbyDelete,
	LobbyMove,

	SavefileSelect,
	SavefileView,
	SavefileEdit,
	SavefileCreate,
	SavefileDelete,
	SavefileMove,

	UserSelect,
	UserView,
	UserEdit,
	UserCreate,
	UserDelete,
	UserMove,

	ModBrowser,
	ModInstall,
	ModManage,
	ModView,

	FindCelesteExecutable,
	ChangeCelesteExecutable,
	EditLaunchOptions,
	
	MapCreationSelect,
	MapCreationCreate,
	
	Tier,
	Achievements,
	Wiki,

	Out
}
