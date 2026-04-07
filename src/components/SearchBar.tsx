interface Props {
  value: string;
  onChange: (value: string) => void;
}

export function SearchBar({ value, onChange }: Props) {
  return (
    <input
      type="text"
      placeholder="Search aliases..."
      value={value}
      onChange={(e) => onChange(e.target.value)}
      style={{
        width: '100%',
        padding: '0.5rem 0.75rem',
        border: '1px solid #ddd',
        borderRadius: '6px',
        fontSize: '0.875rem',
        marginBottom: '0.75rem',
        boxSizing: 'border-box',
      }}
    />
  );
}
