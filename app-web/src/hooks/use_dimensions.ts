import {RefObject, useEffect, useMemo, useState} from "react";
import {ResizeObserver} from "@juggle/resize-observer";
import {useIsomorphicLayoutEffect} from "react-use";

export default function useDimensions<TElement extends Element>(ref: RefObject<TElement>) {
  const [dimensions, setDimensions] = useState<DOMRectReadOnly>(defaultDimensions());

  const observer = useMemo(() =>
    new ResizeObserver((entries) => {
      const dimensions = entries[0]?.contentRect;
      if (dimensions) {
        setDimensions(dimensions);
      }
    }),
    []
  );

  useIsomorphicLayoutEffect(() => {
    if (!ref.current)
      return;

    observer.observe(ref.current);
    return observer.disconnect()
  }, [ref.current]);

  return dimensions;
}

function defaultDimensions(): DOMRectReadOnly {
  return {
    x: 0,
    y: 0,

    top: 0,
    left: 0,
    right: 0,
    bottom: 0,

    width: 0,
    height: 0,

    toJSON(): string {
      return JSON.stringify(this);
    }
  };
}
