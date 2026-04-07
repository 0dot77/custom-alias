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
  const { aliases, loading: aliasesLoading, error, add, update, remove, removeExternal } = useAliases(activeShell);

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

  const [deleteError, setDeleteError] = useState<string | null>(null);
  const [deletingExternal, setDeletingExternal] = useState<MergedAlias | null>(null);

  const handleDelete = async () => {
    if (deletingName) {
      try {
        await remove(deletingName);
        setDeletingName(null);
        setDeleteError(null);
      } catch (e) {
        setDeleteError(String(e));
        setDeletingName(null);
      }
    }
  };

  const handleDeleteExternal = async () => {
    if (deletingExternal && deletingExternal.source.path && deletingExternal.source.line != null) {
      try {
        await removeExternal(deletingExternal.source.path, deletingExternal.source.line);
        setDeletingExternal(null);
        setDeleteError(null);
      } catch (e) {
        setDeleteError(String(e));
        setDeletingExternal(null);
      }
    }
  };

  if (shellsLoading) {
    return (
      <div className="app-container">
        <div className="loading">detecting shells</div>
      </div>
    );
  }

  return (
    <div className="app-container">
      <header className="app-header">
        <h1 className="app-title">custom-alias</h1>
        <button
          className="btn btn-primary"
          onClick={() => {
            setEditingAlias(null);
            setShowForm(true);
          }}
          disabled={!activeShell}
        >
          + add
        </button>
      </header>

      <main className="app-content">
        <ShellSelector shells={shells} activeShell={activeShell} onSelect={setActiveShell} />
        <SearchBar value={search} onChange={setSearch} />

        {error && <div className="error-banner">{error}</div>}
        {deleteError && <div className="error-banner">{deleteError}</div>}

        {aliasesLoading ? (
          <div className="loading">loading aliases</div>
        ) : (
          <>
            <div className="status-bar">
              <span className="status-count">
                {filtered.length} alias{filtered.length !== 1 ? 'es' : ''}
                {search && ` / ${userAliases.length} total`}
              </span>
              {runtimeOnlyCount > 0 && (
                <button
                  className={`btn-toggle ${showAll ? 'active' : ''}`}
                  onClick={() => setShowAll(!showAll)}
                >
                  {showAll ? 'hide' : 'show'} plugins ({runtimeOnlyCount})
                </button>
              )}
            </div>
            <AliasTable aliases={filtered} onEdit={handleEdit} onDelete={setDeletingName} onDeleteExternal={setDeletingExternal} />
          </>
        )}
      </main>

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
          aliasName={deletingName}
          onConfirm={handleDelete}
          onCancel={() => setDeletingName(null)}
        />
      )}

      {deletingExternal && (
        <ConfirmDialog
          aliasName={deletingExternal.name}
          onConfirm={handleDeleteExternal}
          onCancel={() => setDeletingExternal(null)}
        />
      )}
    </div>
  );
}
