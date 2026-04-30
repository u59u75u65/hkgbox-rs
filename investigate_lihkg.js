const { chromium } = require('playwright');

(async () => {
    console.log('🔍 Starting LIHKG API investigation...\n');

    const browser = await chromium.launch({
        headless: false,  // Show the browser
        slowMo: 1000     // Slow down actions for visibility
    });

    const context = await browser.newContext({
        viewport: { width: 1280, height: 720 },
        userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
    });

    // Enable network monitoring
    const page = await context.newPage();

    // Capture all API requests
    page.on('request', request => {
        const url = request.url();
        if (url.includes('api_v2') || url.includes('api')) {
            console.log('📤 API Request:', request.method(), url);
            console.log('   Headers:', JSON.stringify(request.headers(), null, 2));
            const postData = request.postData();
            if (postData && postData.length > 0) {
                console.log('   Post Data:', postData);
            }
            console.log('');
        }
    });

    // Capture all API responses
    page.on('response', async response => {
        const url = response.url();
        if (url.includes('api_v2') || url.includes('api')) {
            console.log('📥 API Response:', response.status(), url);
            try {
                const contentType = response.headers()['content-type'];
                if (contentType && contentType.includes('application/json')) {
                    const json = await response.json();
                    console.log('   Response Data:', JSON.stringify(json, null, 2));

                    // Save important cookies and tokens
                    if (json.success === 1) {
                        console.log('   ✅ Successful API call');
                    }
                } else {
                    console.log('   Content-Type:', contentType);
                }
            } catch (e) {
                console.log('   Response text:', await response.text().substring(0, 200));
            }
            console.log('');
        }
    });

    console.log('🌐 Navigating to LIHKG homepage first...\n');
    await page.goto('https://lihkg.com', { waitUntil: 'networkidle' });

    console.log('\n⏳ Waiting 5 seconds to observe initial API calls...\n');
    await page.waitForTimeout(5000);

    console.log('🎯 Navigating to 吹水台 (category 1)...\n');
    await page.goto('https://lihkg.com/category/1', { waitUntil: 'networkidle' });

    console.log('\n⏳ Waiting 5 seconds to observe category API calls...\n');
    await page.waitForTimeout(5000);

    // Get cookies after page load
    const cookies = await context.cookies();
    console.log('🍪 Cookies after page load:');
    cookies.forEach(cookie => {
        if (cookie.name.includes('cf') || cookie.name.includes('session') || cookie.name.includes('token')) {
            console.log(`   ${cookie.name} = ${cookie.value.substring(0, 20)}...`);
        }
    });
    console.log(`   Total cookies: ${cookies.length}`);
    console.log('');

    // Get localStorage
    const localStorage = await page.evaluate(() => {
        const items = {};
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i);
            items[key] = localStorage.getItem(key);
        }
        return items;
    });
    console.log('💾 LocalStorage items:', Object.keys(localStorage).length);
    Object.keys(localStorage).forEach(key => {
        if (key.includes('token') || key.includes('session') || key.includes('auth')) {
            console.log(`   ${key} = ${localStorage[key].substring(0, 50)}...`);
        }
    });

    console.log('\n⏳ Keeping browser open for 30 seconds for manual investigation...');
    console.log('   You can navigate the page manually to observe more API calls\n');

    await page.waitForTimeout(30000);

    console.log('\n🔍 Investigation complete! Closing browser...');

    await browser.close();
    console.log('✅ Done!');
})();
