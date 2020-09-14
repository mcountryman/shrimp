import {useEffect, useState} from "react";
import sites from "../resources/data/sites.json";

const DEFAULT_INTERVAL = 10_000;

/**
 * Generates random site on interval.
 * @param intervalMs
 * @returns [ randomSite, lastRandomSite ]
 */
export default function useRandomSite(intervalMs = DEFAULT_INTERVAL) {
  const [siteIndex, setSiteIndex] = useState(0);
  const [lastIndex, setLastIndex] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      const length = sites.length;
      const index = Math.floor(Math.random() * length);

      setLastIndex(siteIndex);
      setSiteIndex(index);
    }, intervalMs);

    return () => clearInterval(interval);
  });

  return [sites[siteIndex], sites[lastIndex]];
};
