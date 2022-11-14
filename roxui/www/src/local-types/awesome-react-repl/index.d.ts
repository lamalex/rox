declare module "awesome-react-repl" {
    import React from 'react';

    export interface ReplLine {
        type: "input" | "output" | "error",
        value: string
    };

    export interface ReactReplViewProps {
        title?: string;
        tabs?: string[];
        onSubmit: (string) => void;
        onClear?: (string) => void;
        height?: number,
        lines: Array<ReplLine>,
    };

    export const ReactReplView: React.FunctionComponent<ReactReplViewProps>;
}