// Monaco Editor Language Definition for .jucode
export function registerJucodeLanguage(monaco) {
    if (!monaco) return;

    monaco.languages.register({ id: 'jucode' });

    monaco.languages.setMonarchTokensProvider('jucode', {
        keywords: [
            'fn', 'let', 'mut', 'struct', 'async', 'await',
            'return', 'if', 'else', 'match', 'spawn', 'import', 'export'
        ],
        typeKeywords: ['int', 'float', 'string', 'bool', 'void', 'Object', 'Array'],
        operators: ['=', '==', '!=', '+', '-', '*', '/', '%', '->', '=>'],

        tokenizer: {
            root: [
                [/[a-zA-Z_]\w*/, {
                    cases: {
                        '@keywords': 'keyword',
                        '@typeKeywords': 'type',
                        '@default': 'identifier'
                    }
                }],
                [/"([^"\\]|\\.)*"/, 'string'],
                [/\d+/, 'number'],
                [/\/\/.*/, 'comment'],
            ]
        }
    });
}
