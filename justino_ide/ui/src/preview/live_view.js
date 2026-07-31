// Live Preview Panel Engine for .jucode + .css UI Windows
export class LivePreviewEngine {
    constructor(iframeId) {
        this.iframe = document.getElementById(iframeId);
    }

    updatePreview(jucodeContent, cssContent) {
        if (!this.iframe) return;
        const html = `
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body { margin: 0; padding: 16px; background-color: #f4f6f9; font-family: sans-serif; }
                    ${cssContent}
                </style>
            </head>
            <body>
                <window id="app-window">
                    <div class="header-container">
                        <text id="titulo" class="title-app">Live Preview: Justino GUI Engine</text>
                    </div>
                    <div class="form-box" style="margin-top: 16px;">
                        <input class="input-text" type="text" value="Live Preview Active" />
                        <button class="btn-action">Action</button>
                    </div>
                </window>
            </body>
            </html>
        `;
        this.iframe.srcdoc = html;
    }
}
