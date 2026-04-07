export type ShellType = 'bash' | 'zsh' | 'fish' | 'powerShell';

export interface DetectedShell {
  shellType: ShellType;
  binaryPath: string;
  configFiles: string[];
  isDefault: boolean;
}

export interface AliasSource {
  type: 'configFile' | 'runtimeOnly' | 'both';
  path?: string;
  line?: number;
}

export interface MergedAlias {
  name: string;
  command: string;
  shell: ShellType;
  source: AliasSource;
  group: string | null;
  isManaged: boolean;
}

export interface AliasInput {
  name: string;
  command: string;
  shell: ShellType;
  group: string | null;
}

export interface BackupInfo {
  path: string;
  shell: ShellType;
  createdAt: string;
  originalFile: string;
}
