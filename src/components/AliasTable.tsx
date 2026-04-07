import type { MergedAlias } from '../lib/types';

interface Props {
  aliases: MergedAlias[];
  onEdit: (alias: MergedAlias) => void;
  onDelete: (name: string) => void;
}

function sourceBadge(alias: MergedAlias): { label: string; className: string } {
  if (alias.isManaged) return { label: 'managed', className: 'badge badge-managed' };
  switch (alias.source.type) {
    case 'runtimeOnly':
      return { label: 'plugin', className: 'badge badge-runtime' };
    default:
      return { label: 'config', className: 'badge badge-config' };
  }
}

export function AliasTable({ aliases, onEdit, onDelete }: Props) {
  if (aliases.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-state-icon">~</div>
        <div>No aliases found</div>
      </div>
    );
  }

  return (
    <div style={{ overflowX: 'auto' }}>
      <table className="alias-table">
        <thead>
          <tr>
            <th style={{ width: '15%' }}>name</th>
            <th style={{ width: '38%' }}>command</th>
            <th style={{ width: '10%' }}>group</th>
            <th style={{ width: '10%' }}>source</th>
            <th style={{ width: '15%' }}>file</th>
            <th style={{ width: '12%' }}>actions</th>
          </tr>
        </thead>
        <tbody>
          {aliases.map((alias) => {
            const badge = sourceBadge(alias);
            return (
              <tr key={alias.name} className="alias-row">
                <td><span className="alias-name">{alias.name}</span></td>
                <td><span className="alias-command">{alias.command}</span></td>
                <td>
                  {alias.group && (
                    <span className="badge badge-group">{alias.group}</span>
                  )}
                </td>
                <td><span className={badge.className}>{badge.label}</span></td>
                <td>
                  <span className="badge-source-file">
                    {alias.source.path ? alias.source.path.split('/').pop() : '-'}
                  </span>
                </td>
                <td>
                  {alias.isManaged ? (
                    <div className="action-buttons">
                      <button className="btn-ghost" onClick={() => onEdit(alias)}>
                        edit
                      </button>
                      <button className="btn-danger" onClick={() => onDelete(alias.name)}>
                        del
                      </button>
                    </div>
                  ) : (
                    <span className="read-only-label">readonly</span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
