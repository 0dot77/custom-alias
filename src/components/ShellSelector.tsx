import type { DetectedShell, ShellType } from '../lib/types';

const SHELL_LABELS: Record<ShellType, string> = {
  bash: 'bash',
  zsh: 'zsh',
  fish: 'fish',
  powerShell: 'pwsh',
};

interface Props {
  shells: DetectedShell[];
  activeShell: ShellType | null;
  onSelect: (shell: ShellType) => void;
}

export function ShellSelector({ shells, activeShell, onSelect }: Props) {
  return (
    <div className="shell-tabs">
      {shells.map((s) => (
        <button
          key={s.shellType}
          onClick={() => onSelect(s.shellType)}
          className={`shell-tab ${activeShell === s.shellType ? 'active' : ''}`}
        >
          {SHELL_LABELS[s.shellType]}
          {s.isDefault && <span className="default-dot" />}
        </button>
      ))}
    </div>
  );
}
