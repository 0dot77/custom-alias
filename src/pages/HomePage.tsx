import { useState } from 'react';
import type { MergedAlias, AliasInput } from '../lib/types';
import { useShells } from '../hooks/useShells';
import { useAliases } from '../hooks/useAliases';
import { ShellSelector } from '../components/ShellSelector';
import { SearchBar } from '../components/SearchBar';
import { AliasTable } from '../components/AliasTable';
import { AliasForm } from '../components/AliasForm';
import { ConfirmDialog } from '../components/ConfirmDialog';

export function HomePage() {
  const { shells, activeShell, setActiveShell, loading: shellsLoading } = useShells();
  const { aliases, loading: aliasesLoading, error, refresh, add, update, remove } = useAliases(activeShell);

  const [search, setSearch] = useState('');
  const [showForm, setShowForm] = useState(false);
  const [editingAlias, setEditingAlias] = useState<MergedAlias | null>(null);
  const [deletingName, setDeletingName] = useState<string | null>(null);
  const [showAll, setShowAll] = useState(false);

  const userAliases = showAll
    ? aliases
    : aliases.filter((a) => a.source.type !== 'runtimeOnly');

  const filtered = userAliases.filter(
    (a) =>
      a.name.toLowerCase().includes(search.toLowerCase()) ||
      a.command.toLowerCase().includes(search.toLowerCase()) ||
      (a.group ?? '').toLowerCase().includes(search.toLowerCase()),
  );

  const runtimeOnlyCount = aliases.filter((a) => a.source.type === 'runtimeOnly').length;

  const handleSubmit = async (input: AliasInput, oldName?: string) => {
    if (oldName) {
      await update(oldName, input);
    } else {
      await add(input);
    }
    setShowForm(false);
    setEditingAlias(null);
  };

  const handleEdit = (alias: MergedAlias) => {
    setEditingAlias(alias);
    setShowForm(true);
  };

  const handleDelete = async () => {
    if (deletingName) {
      await remove(deletingName);
      setDeletingName(null);
    }
  };

  if (shellsLoading) {
    return <div style={{ padding: '2rem', color: '#999' }}>Detecting shells...</div>;
  }

  return (
    <div style={{ padding: '1.25rem', fontFamily: 'system-ui, -apple-system, sans-serif', maxWidth: '960px', margin: '0 auto' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
        <h1 style={{ margin: 0, fontSize: '1.25rem', fontWeight: 600 }}>Custom Alias</h1>
        <button
          onClick={() => {
            setEditingAlias(null);
            setShowForm(true);
          }}
          disabled={!activeShell}
          style={{
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '6px',
            background: '#0066ff',
            color: '#fff',
            cursor: 'pointer',
            fontSize: '0.8125rem',
            fontWeight: 500,
          }}
        >
          + Add Alias
        </button>
      </div>

      <ShellSelector shells={shells} activeShell={activeShell} onSelect={setActiveShell} />
      <SearchBar value={search} onChange={setSearch} />

      {error && (
        <div style={{ padding: '0.75rem', background: '#ffebee', color: '#c62828', borderRadius: '6px', marginBottom: '0.75rem', fontSize: '0.8125rem' }}>
          {error}
        </div>
      )}

      {aliasesLoading ? (
        <div style={{ padding: '2rem', textAlign: 'center', color: '#999' }}>Loading aliases...</div>
      ) : (
        <>
          <div style={{ marginBottom: '0.5rem', fontSize: '0.75rem', color: '#999', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <span>
              {filtered.length} alias{filtered.length !== 1 ? 'es' : ''} found
              {search && ` (filtered from ${userAliases.length})`}
            </span>
            {runtimeOnlyCount > 0 && (
              <button
                onClick={() => setShowAll(!showAll)}
                style={{
                  padding: '0.25rem 0.5rem',
                  border: '1px solid #ddd',
                  borderRadius: '4px',
                  background: showAll ? '#fff3e0' : '#f5f5f5',
                  cursor: 'pointer',
                  fontSize: '0.75rem',
                  color: '#666',
                }}
              >
                {showAll ? `Hide plugin aliases (${runtimeOnlyCount})` : `Show plugin aliases (${runtimeOnlyCount})`}
              </button>
            )}
          </div>
          <AliasTable aliases={filtered} onEdit={handleEdit} onDelete={setDeletingName} />
        </>
      )}

      {showForm && activeShell && (
        <AliasForm
          shell={activeShell}
          editingAlias={editingAlias}
          onSubmit={handleSubmit}
          onCancel={() => {
            setShowForm(false);
            setEditingAlias(null);
          }}
        />
      )}

      {deletingName && (
        <ConfirmDialog
          message={`Delete alias "${deletingName}"?`}
          onConfirm={handleDelete}
          onCancel={() => setDeletingName(null)}
        />
      )}
    </div>
  );
}
