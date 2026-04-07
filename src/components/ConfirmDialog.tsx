interface Props {
  aliasName: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ aliasName, onConfirm, onCancel }: Props) {
  return (
    <div className="modal-overlay" onClick={onCancel}>
      <div className="modal-panel" onClick={(e) => e.stopPropagation()}>
        <h3 className="modal-title">› confirm delete</h3>
        <p className="confirm-text">
          Remove alias <span className="confirm-alias-name">{aliasName}</span> from
          the managed section? This can be undone via backup restore.
        </p>
        <div className="modal-actions">
          <button className="btn btn-ghost" onClick={onCancel}>
            cancel
          </button>
          <button className="btn btn-danger" onClick={onConfirm}>
            delete
          </button>
        </div>
      </div>
    </div>
  );
}
