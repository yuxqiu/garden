browser.webRequest.onBeforeRequest.addListener(
    (details) => {
        if (details.type !== "stylesheet") return {};

        const filter = browser.webRequest.filterResponseData(details.requestId);
        const decoder = new TextDecoder("utf-8");
        const encoder = new TextEncoder();
        let input = [];

        filter.ondata = (event) => {
            input.push(decoder.decode(event.data, { stream: true }));
        };

        filter.onstop = (event) => {
            input.push(decoder.decode()); // End of stream
            let str = input.join("");

            // Modify CSS: Replace prefers-color-scheme: light with dark
            str = str.replaceAll(/prefers-color-scheme\s*:\s*light/g, 'prefers-color-scheme: light_temp');
            str = str.replaceAll(/prefers-color-scheme\s*:\s*dark/g, 'prefers-color-scheme: light');
            str = str.replaceAll('prefers-color-scheme: light_temp', 'prefers-color-scheme: dark');

            filter.write(encoder.encode(str));
            filter.close();
        };

        filter.onerror = (event) => {
            console.error(`Filter error for ${details.url}: ${filter.error}`);
            filter.close();
        };

        return {};
    },
    { urls: ["<all_urls>"], types: ["stylesheet"] },
    ["blocking"]
);
