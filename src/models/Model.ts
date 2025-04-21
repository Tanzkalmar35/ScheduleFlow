export interface Model {
	readonly shadowType: ShadowType;
}

export enum ShadowType {
	Calendar,
	Component,
	Property,
	User
}
