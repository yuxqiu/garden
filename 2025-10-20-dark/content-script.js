(function () {
    // Set to store modified rules
    const modifiedRules = new Set();

    // Remove integrity attributes from <link>
    function removeIntegrityAttributes() {
        const links = document.querySelectorAll('link[rel="stylesheet"][integrity]');
        links.forEach(link => {
            console.log(`Removing integrity attribute for ${link.href}`);
            link.removeAttribute('integrity');
        });
    }

    // Modify stylesheets (skip Blob URLs)
    function modifyStylesheets() {
        const styleSheets = document.styleSheets;
        for (let sheet of styleSheets) {
            if (sheet.href && sheet.href !== null) {
                console.log('Skipping non-inline stylesheet:', sheet.href);
                continue;
            }

            try {
                const rules = sheet.cssRules; // this can fail when accessing some stylesheet without css rules
                for (let i = rules.length - 1; i >= 0; i--) {
                    const rule = rules[i];
                    if (rule instanceof CSSMediaRule) {
                        if (rule.media.mediaText.includes(/prefers-color-scheme\s*:\s*light/g) && !modifiedRules.has(rule.cssText)) {
                            console.log('Removed original light rule:', rule.cssText);
                            sheet.deleteRule(i);
                        } else if (rule.media.mediaText.includes(/prefers-color-scheme\s*:\s*dark/g)) {
                            const newRuleText = rule.cssText.replace(/prefers-color-scheme\s*:\s*dark/g, 'prefers-color-scheme: light');
                            console.log('Old rule:', rule.cssText);
                            console.log('New rule:', newRuleText);
                            modifiedRules.add(newRuleText);
                            sheet.deleteRule(i);
                            sheet.insertRule(newRuleText, i);
                        }
                    }
                }
            } catch (e) {
                console.warn('Cannot access stylesheet:', {
                    href: sheet.href || 'inline stylesheet',
                    error: e.message
                });
            }
        }
    }

    // Run immediately
    removeIntegrityAttributes();
    modifyStylesheets();

    // Observe for new <link> or <style>
    let debounceTimeout;
    const observer = new MutationObserver(() => {
        clearTimeout(debounceTimeout);
        debounceTimeout = setTimeout(() => {
            console.log('MutationObserver triggered: New elements detected');
            removeIntegrityAttributes();
            modifyStylesheets();
        }, 100);
    });
    observer.observe(document.documentElement, {
        childList: true,
        subtree: true
    });

    const originalMatchMedia = window.matchMedia;
    window.matchMedia = function (query) {
        if (query.includes('prefers-color-scheme')) {
            return {
                matches: query.includes('dark'),
                media: query,
                addListener: () => { },
                removeListener: () => { }
            };
        }
        return originalMatchMedia(query);
    };
})();
