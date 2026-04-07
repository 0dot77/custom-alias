import { invoke } from '@tauri-apps/api/core';
import type { DetectedShell, MergedAlias, AliasInput, BackupInfo, ShellType } from './types';

export async function detectShells(): Promise<DetectedShell[]> {
  return invoke('detect_shells');
}

export async function getAliases(shell: ShellType): Promise<MergedAlias[]> {
  return invoke('get_aliases', { shell });
}

export async function addAlias(input: AliasInput): Promise<MergedAlias> {
  return invoke('add_alias', { input });
}

export async function updateAlias(oldName: string, input: AliasInput): Promise<MergedAlias> {
  return invoke('update_alias', { oldName, input });
}

export async function deleteAlias(name: string, shell: ShellType): Promise<void> {
  return invoke('delete_alias', { name, shell });
}

export async function deleteExternalAlias(filePath: string, line: number, shell: ShellType): Promise<void> {
  return invoke('delete_external_alias', { filePath, line, shell });
}

export async function suppressAlias(name: string, shell: ShellType): Promise<void> {
  return invoke('suppress_alias', { name, shell });
}

export async function importAlias(name: string, shell: ShellType): Promise<MergedAlias> {
  return invoke('import_alias', { name, shell });
}

export async function listBackups(shell: ShellType): Promise<BackupInfo[]> {
  return invoke('list_backups', { shell });
}

export async function restoreBackup(backupPath: string): Promise<void> {
  return invoke('restore_backup', { backupPath });
}
