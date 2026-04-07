interface Props {
  value: string;
  onChange: (value: string) => void;
}

export function SearchBar({ value, onChange }: Props) {
  return (
    <div className="search-container">
      <span className="search-icon">/</span>
      <input
        type="text"
        className="search-input"
        placeholder="search aliases..."
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
    </div>
  );
}
