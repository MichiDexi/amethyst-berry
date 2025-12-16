#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
	Main,

	MapSelect,
	MapView,
	MapEdit,
	MapCreate,
	MapDelete,
	MapRename,
	MapPractice,

	LobbySelect,
	LobbyView,
	LobbyEdit,
	LobbyCreate,
	LobbyDelete,
	LobbyRename,

	SavefileSelect,
	SavefileView,
	SavefileEdit,
	SavefileCreate,
	SavefileDelete,
	SavefileRename,

	UserSelect,
	UserView,
	UserEdit,
	UserCreate,
	UserDelete,
	UserRename,

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
