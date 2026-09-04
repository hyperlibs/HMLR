import { describe, it } from 'node:test';
import * as assert from 'node:assert';
import * as fs from 'node:fs';
import * as path from 'node:path';
import { HMLRRunner } from '../src/bootstrap/runner.js';

describe('HMLR .mx & .fx Architecture Verification', () => {
  it('should parse MANIFEST.mx correctly', () => {
    const manifestPath = path.resolve('MANIFEST.mx');
    const content = fs.readFileSync(manifestPath, 'utf-8');
    const doc = HMLRRunner.parseMX(content);

    assert.strictEqual(doc.meta['project'], 'HMLR');
    assert.strictEqual(doc.meta['runtime'], 'hmlr@1.0');
    assert.ok(doc.tables.length >= 2, 'Should parse SubsystemScope and BusProtocol tables');

    const scopeTable = doc.tables.find(t => t.model === 'SubsystemScope');
    assert.ok(scopeTable, 'SubsystemScope table must exist');
    assert.ok(scopeTable.rows.length >= 6, 'Should have all 6 core subsystems');
  });

  it('should parse src/syntax/mx_grammar.mx', () => {
    const filePath = path.resolve('src/syntax/mx_grammar.mx');
    const content = fs.readFileSync(filePath, 'utf-8');
    const doc = HMLRRunner.parseMX(content);

    assert.strictEqual(doc.meta['version'], '"1.0.0"');
    const grammarTable = doc.tables.find(t => t.model === 'GrammarProduction');
    assert.ok(grammarTable, 'GrammarProduction table must exist');
    assert.ok(grammarTable.rows.length >= 5);
  });

  it('should parse src/diagnostics/codes.mx', () => {
    const filePath = path.resolve('src/diagnostics/codes.mx');
    const content = fs.readFileSync(filePath, 'utf-8');
    const doc = HMLRRunner.parseMX(content);

    assert.strictEqual(doc.meta['prefix'], '"FX"');
    const diagTable = doc.tables.find(t => t.model === 'DiagnosticSpec');
    assert.ok(diagTable, 'DiagnosticSpec table must exist');
    assert.ok(diagTable.rows.some(r => r.code === 'FX-2001'), 'Must include FX-2001 boundary error');
    assert.ok(diagTable.rows.some(r => r.code === 'FX-2003'), 'Must include FX-2003 JSON prohibition');
  });

  it('should verify all core .fx source files exist', () => {
    const fxFiles = [
      'src/compiler/tokenizer.fx',
      'src/compiler/ast.fx',
      'src/compiler/hir.fx',
      'src/diagnostics/formatter.fx'
    ];

    for (const file of fxFiles) {
      assert.ok(fs.existsSync(path.resolve(file)), `Expected ${file} to exist`);
      const content = fs.readFileSync(path.resolve(file), 'utf-8');
      assert.ok(content.length > 50, `${file} should contain .fx code`);
    }
  });
});
