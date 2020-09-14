import {RefObject} from "react";
import useDimensions from "../../hooks/use_dimensions";

interface IClipboardTooltipProps<TAnchor extends Element> {
  anchor: RefObject<TAnchor>;
}

export default function ClipboardTooltip<TAnchor extends Element>(
  props: IClipboardTooltipProps<TAnchor>
) {
  const dimensions = useDimensions(props.anchor);

  return (
    <div>

    </div>
  );
}
