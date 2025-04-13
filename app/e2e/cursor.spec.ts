import { test, expect } from '@playwright/test';

test.describe('カーソル移動のテスト', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:1420');
  });

  test('カーソルを下に移動', async ({ page }) => {
    // カーソル移動ボタンをクリック
    await page.click('button:has-text("カーソルを下に移動")');
    
    // 結果を確認
    const result = await page.textContent('#result');
    expect(result).toContain('カーソルを移動しました');
  });
}); 