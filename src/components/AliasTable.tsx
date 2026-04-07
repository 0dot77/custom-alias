import type { MergedAlias } from '../lib/types';

interface Props {
  aliases: MergedAlias[];
  onEdit: (alias: MergedAlias) => void;
  onDelete: (name: string) => void;
}

function sourceBadge(alias: MergedAlias) {
  if (alias.isManaged) return 'managed';
  switch (alias.source.type) {
    case 'runtimeOnly':
      return 'runtime';
    case 'configFile':
      return 'config';
    case 'both':
      return 'config';
  }
}

function badgeColor(badge: string) {
  switch (badge) {
    case 'managed':
      return { bg: '#e8f5e9', color: '#2e7d32' };
    case 'runtime':
      return { bg: '#fff3e0', color: '#e65100' };
    default:
      return { bg: '#e3f2fd', color: '#1565c0' };
  }
}

const cellStyle: React.CSSProperties = {
  padding: '0.5rem 0.75rem',
  borderBottom: '1px solid #eee',
  fontSize: '0.8125rem',
  textAlign: 'left',
};

export function AliasTable({ aliases, onEdit, onDelete }: Props) {
  if (aliases.length === 0) {
    return (
      <div style={{ padding: '2rem', textAlign: 'center', color: '#999' }}>
        No aliases found.
      </div>
    );
  }

  return (
    <div style={{ overflowX: 'auto' }}>
      <table style={{ width: '100%', borderCollapse: 'collapse' }}>
        <thead>
          <tr style={{ borderBottom: '2px solid #ddd' }}>
            <th style={{ ...cellStyle, fontWeight: 600, width: '15%' }}>Name</th>
            <th style={{ ...cellStyle, fontWeight: 600, width: '40%' }}>Command</th>
            <th style={{ ...cellStyle, fontWeight: 600, width: '10%' }}>Group</th>
            <th style={{ ...cellStyle, fontWeight: 600, width: '10%' }}>Source</th>
            <th style={{ ...cellStyle, fontWeight: 600, width: '15%' }}>File</th>
            <th style={{ ...cellStyle, fontWeight: 600, width: '10%' }}>Actions</th>
          </tr>
        </thead>
        <tbody>
          {aliases.map((alias) => {
            const badge = sourceBadge(alias);
            const colors = badgeColor(badge);
            return (
              <tr key={alias.name} style={{ transition: 'background 0.15s' }}>
                <td style={{ ...cellStyle, fontWeight: 500, fontFamily: 'monospace' }}>
                  {alias.name}
                </td>
                <td style={{ ...cellStyle, fontFamily: 'monospace', color: '#555' }}>
                  {alias.command}
                </td>
                <td style={cellStyle}>
                  {alias.group && (
                    <span
                      style={{
                        padding: '0.125rem 0.5rem',
                        borderRadius: '9999px',
                        background: '#f3e8ff',
                        color: '#7c3aed',
                        fontSize: '0.75rem',
                      }}
                    >
                      {alias.group}
                    </span>
                  )}
                </td>
                <td style={cellStyle}>
                  <span
                    style={{
                      padding: '0.125rem 0.5rem',
                      borderRadius: '9999px',
                      background: colors.bg,
                      color: colors.color,
                      fontSize: '0.75rem',
                    }}
                  >
                    {badge}
                  </span>
                </td>
                <td style={{ ...cellStyle, fontSize: '0.75rem', color: '#888' }}>
                  {alias.source.path
                    ? alias.source.path.split('/').pop()
                    : '-'}
                </td>
                <td style={cellStyle}>
                  {alias.isManaged ? (
                    <span style={{ display: 'flex', gap: '0.25rem' }}>
                      <button
                        onClick={() => onEdit(alias)}
                        style={{
                          padding: '0.25rem 0.5rem',
                          border: '1px solid #ddd',
                          borderRadius: '4px',
                          background: '#fff',
                          cursor: 'pointer',
                          fontSize: '0.75rem',
                        }}
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => onDelete(alias.name)}
                        style={{
                          padding: '0.25rem 0.5rem',
                          border: '1px solid #ffcdd2',
                          borderRadius: '4px',
                          background: '#fff',
                          color: '#c62828',
                          cursor: 'pointer',
                          fontSize: '0.75rem',
                        }}
                      >
                        Del
                      </button>
                    </span>
                  ) : (
                    <span style={{ color: '#aaa', fontSize: '0.75rem' }}>read-only</span>
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
