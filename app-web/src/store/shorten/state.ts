
export enum ShortenState {
  Ready,
  Error,
  Shortened,
}

export interface IShortenState {
  state: ShortenState,

  long?: string,
  short?: string,
  error?: Error,
}

export const defaultShortenState: IShortenState = {
  long: "",
  state: ShortenState.Ready,
};
