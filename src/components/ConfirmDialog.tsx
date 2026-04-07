interface Props {
  message: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ message, onConfirm, onCancel }: Props) {
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
      <div
        onClick={(e) => e.stopPropagation()}
        style={{
          background: '#fff',
          borderRadius: '12px',
          padding: '1.5rem',
          width: '320px',
          maxWidth: '90vw',
          boxShadow: '0 8px 32px rgba(0,0,0,0.15)',
        }}
      >
        <p style={{ margin: '0 0 1rem', fontSize: '0.875rem' }}>{message}</p>
        <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'flex-end' }}>
          <button
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
            onClick={onConfirm}
            style={{
              padding: '0.5rem 1rem',
              border: 'none',
              borderRadius: '6px',
              background: '#c62828',
              color: '#fff',
              cursor: 'pointer',
              fontSize: '0.8125rem',
            }}
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  );
}
