export interface Model {
	readonly shadowType: ShadowType;

	getShadowType(): ShadowType;
}

export enum ShadowType {
	Calendar,
	Component,
	Property,
	User
}
