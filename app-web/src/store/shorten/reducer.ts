import { AppAction } from "../action";
import { reduceShortenError, SHORTEN_ERROR } from "./actions/shorten_error";
import { SHORTEN_LINK } from "./actions/shorten_link";
import { IShortenState } from "./state";

export function reduceShorten(state: IShortenState, action: AppAction) {
  switch (action.type) {
    case SHORTEN_LINK:
      break;
    case SHORTEN_ERROR:
      return Object.assign({}, state, reduceShortenError(action));
    default:
      return state;
  }
}
