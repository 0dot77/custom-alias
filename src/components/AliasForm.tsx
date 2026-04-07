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

  return (
    <div className="modal-overlay" onClick={onCancel}>
      <form
        onSubmit={handleSubmit}
        onClick={(e) => e.stopPropagation()}
        className="modal-panel"
      >
        <h3 className="modal-title">
          {editingAlias ? '› edit alias' : '› new alias'}
        </h3>

        <div className="field">
          <label className="field-label">name</label>
          <input
            className="field-input"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="gs"
            autoFocus
          />
        </div>
        <div className="field">
          <label className="field-label">command</label>
          <input
            className="field-input"
            value={command}
            onChange={(e) => setCommand(e.target.value)}
            placeholder="git status"
          />
        </div>
        <div className="field">
          <label className="field-label">
            group <span className="optional">(optional)</span>
          </label>
          <input
            className="field-input"
            value={group}
            onChange={(e) => setGroup(e.target.value)}
            placeholder="git"
          />
        </div>

        {error && <div className="form-error">{error}</div>}

        <div className="modal-actions">
          <button type="button" className="btn btn-ghost" onClick={onCancel}>
            cancel
          </button>
          <button
            type="submit"
            className="btn btn-primary"
            disabled={submitting || !name.trim() || !command.trim()}
            style={{ opacity: submitting ? 0.6 : 1 }}
          >
            {submitting ? 'saving...' : editingAlias ? 'update' : 'add'}
          </button>
        </div>
      </form>
    </div>
  );
}
