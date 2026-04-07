import type { DetectedShell, ShellType } from '../lib/types';

const SHELL_LABELS: Record<ShellType, string> = {
  bash: 'Bash',
  zsh: 'Zsh',
  fish: 'Fish',
  powerShell: 'PowerShell',
};

interface Props {
  shells: DetectedShell[];
  activeShell: ShellType | null;
  onSelect: (shell: ShellType) => void;
}

export function ShellSelector({ shells, activeShell, onSelect }: Props) {
  return (
    <div style={{ display: 'flex', gap: '0.25rem', marginBottom: '1rem' }}>
      {shells.map((s) => (
        <button
          key={s.shellType}
          onClick={() => onSelect(s.shellType)}
          style={{
            padding: '0.5rem 1rem',
            border: '1px solid #ccc',
            borderRadius: '6px',
            background: activeShell === s.shellType ? '#0066ff' : '#f5f5f5',
            color: activeShell === s.shellType ? '#fff' : '#333',
            cursor: 'pointer',
            fontWeight: activeShell === s.shellType ? 600 : 400,
            fontSize: '0.875rem',
          }}
        >
          {SHELL_LABELS[s.shellType]}
          {s.isDefault && ' *'}
        </button>
      ))}
    </div>
  );
}
