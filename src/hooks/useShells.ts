import { useEffect, useState } from 'react';
import type { DetectedShell, ShellType } from '../lib/types';
import { detectShells } from '../lib/tauri';

export function useShells() {
  const [shells, setShells] = useState<DetectedShell[]>([]);
  const [activeShell, setActiveShell] = useState<ShellType | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    detectShells()
      .then((detected) => {
        setShells(detected);
        const defaultShell = detected.find((s) => s.isDefault) ?? detected[0];
        if (defaultShell) {
          setActiveShell(defaultShell.shellType);
        }
      })
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }, []);

  return { shells, activeShell, setActiveShell, loading, error };
}
