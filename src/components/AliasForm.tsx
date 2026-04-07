import { useState, useEffect } from 'react';
import type { MergedAlias, ShellType, AliasInput } from '../lib/types';

interface Props {
  shell: ShellType;
  editingAlias: MergedAlias | null;
  onSubmit: (input: AliasInput, oldName?: string) => Promise<void>;
  onCancel: () => void;
}

export function AliasForm({ shell, editingAlias, onSubmit, onCancel }: Props) {
  const [name, setName] = useState('');
  const [command, setCommand] = useState('');
  const [group, setGroup] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (editingAlias) {
      setName(editingAlias.name);
      setCommand(editingAlias.command);
      setGroup(editingAlias.group ?? '');
    } else {
      setName('');
      setCommand('');
      setGroup('');
    }
  }, [editingAlias]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !command.trim()) return;

    setSubmitting(true);
    setError(null);
    try {
      const input: AliasInput = {
        name: name.trim(),
        command: command.trim(),
        shell,
        group: group.trim() || null,
      };
      await onSubmit(input, editingAlias?.name);
      setName('');
      setCommand('');
      setGroup('');
    } catch (e) {
      setError(String(e));
    } finally {
      setSubmitting(false);
    }
  };

  const inputStyle: React.CSSProperties = {
    padding: '0.5rem 0.75rem',
    border: '1px solid #ddd',
    borderRadius: '6px',
    fontSize: '0.875rem',
    width: '100%',
    boxSizing: 'border-box',
  };

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.3)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 100,
      }}
      onClick={onCancel}
    >
      <form
        onSubmit={handleSubmit}
        onClick={(e) => e.stopPropagation()}
        style={{
          background: '#fff',
          borderRadius: '12px',
          padding: '1.5rem',
          width: '400px',
          maxWidth: '90vw',
          boxShadow: '0 8px 32px rgba(0,0,0,0.15)',
        }}
      >
        <h3 style={{ margin: '0 0 1rem', fontSize: '1rem' }}>
          {editingAlias ? 'Edit Alias' : 'Add Alias'}
        </h3>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
          <div>
            <label style={{ fontSize: '0.8125rem', fontWeight: 500 }}>Name</label>
            <input
              style={inputStyle}
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. gs"
              autoFocus
            />
          </div>
          <div>
            <label style={{ fontSize: '0.8125rem', fontWeight: 500 }}>Command</label>
            <input
              style={inputStyle}
              value={command}
              onChange={(e) => setCommand(e.target.value)}
              placeholder="e.g. git status"
            />
          </div>
          <div>
            <label style={{ fontSize: '0.8125rem', fontWeight: 500 }}>
              Group <span style={{ color: '#999' }}>(optional)</span>
            </label>
            <input
              style={inputStyle}
              value={group}
              onChange={(e) => setGroup(e.target.value)}
              placeholder="e.g. git"
            />
          </div>
        </div>

        {error && (
          <div style={{ marginTop: '0.75rem', color: '#c62828', fontSize: '0.8125rem' }}>
            {error}
          </div>
        )}

        <div style={{ display: 'flex', gap: '0.5rem', marginTop: '1rem', justifyContent: 'flex-end' }}>
          <button
            type="button"
            onClick={onCancel}
            style={{
              padding: '0.5rem 1rem',
              border: '1px solid #ddd',
              borderRadius: '6px',
              background: '#f5f5f5',
              cursor: 'pointer',
              fontSize: '0.8125rem',
            }}
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={submitting || !name.trim() || !command.trim()}
            style={{
              padding: '0.5rem 1rem',
              border: 'none',
              borderRadius: '6px',
              background: '#0066ff',
              color: '#fff',
              cursor: 'pointer',
              fontSize: '0.8125rem',
              opacity: submitting ? 0.6 : 1,
            }}
          >
            {submitting ? 'Saving...' : editingAlias ? 'Update' : 'Add'}
          </button>
        </div>
      </form>
    </div>
  );
}
