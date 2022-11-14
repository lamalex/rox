import { useState } from 'react';
import { ReactReplView, ReplLine } from "awesome-react-repl";
import init, { run_code, set_debug } from "roxui";

const GeneralPurposeReplUI = () => {
  const [lines, setLines] = useState<ReplLine[]>([]);

  const update = async (line: string) => {
    line = line.trimEnd();

    const newLines: Array<ReplLine> = [
      {
        type: "input",
        value: line
      }
    ];

    if (line === '') {

      let source = lines
        .filter(line => line.type === "input")
        .filter(line => line.value.length > 0)
        .map(line => line.value)
        .join('\n');

      await init();
      await set_debug();

      try {
        const output = await run_code(source);
        newLines.push({
          type: "output",
          value: output
        });
      } catch (e: any) {
        newLines.push({
          type: "error",
          value: `line ${e.line}: ${e.source}`
        })
      }
    }

    setLines([...lines, ...newLines]);
  }

  return (
    <ReactReplView
      title={`launi's rox`}
      tabs={["rox"]}
      onSubmit={update}
      onClear={async () => await setLines([])}
      height={window.screen.height * 0.8}
      lines={lines}
    />
  )
}

export default GeneralPurposeReplUI;