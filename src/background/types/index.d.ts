export interface ConfigSettingTypes {
	autoPickChampion: { championId: string; isAuto: boolean };
	autoBanChampion: { championId: string; isAuto: boolean };
	autoIsOne: boolean;
	autoAccept: number;
	theme: string;
	isGameInWindow: boolean;
	isGameInTips: boolean;
	autoWriteBlock: boolean;
	inWinOpacity: number;
	warmTips: {
		autoRune: boolean;
		rankTips: boolean;
		teamTips: boolean;
	};
	lolTracker: number;
	shouldCloseLOL: boolean;
}
export interface ConfigRank {
	tier: number;
	lane: string;
	is101: boolean;
}