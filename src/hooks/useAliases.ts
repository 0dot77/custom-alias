import { useCallback, useEffect, useState } from 'react';
import type { MergedAlias, ShellType, AliasInput } from '../lib/types';
import { getAliases, addAlias, updateAlias, deleteAlias, deleteExternalAlias, suppressAlias } from '../lib/tauri';

export function useAliases(shell: ShellType | null) {
  const [aliases, setAliases] = useState<MergedAlias[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    if (!shell) return;
    setLoading(true);
    setError(null);
    try {
      const result = await getAliases(shell);
      setAliases(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [shell]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const add = async (input: AliasInput) => {
    await addAlias(input);
    await refresh();
  };

  const update = async (oldName: string, input: AliasInput) => {
    await updateAlias(oldName, input);
    await refresh();
  };

  const remove = async (name: string) => {
    if (!shell) return;
    await deleteAlias(name, shell);
    await refresh();
  };

  const removeExternal = async (filePath: string, line: number) => {
    if (!shell) return;
    await deleteExternalAlias(filePath, line, shell);
    await refresh();
  };

  const suppress = async (name: string) => {
    if (!shell) return;
    await suppressAlias(name, shell);
    await refresh();
  };

  return { aliases, loading, error, refresh, add, update, remove, removeExternal, suppress };
}
