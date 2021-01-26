import React, { ReactElement } from 'react';
import { AnswerlessFlag } from '../../utils/types';

interface Props {
  flag: AnswerlessFlag
}

export function Flag({ flag }: Props): ReactElement {
  return (
    <div>
      <h3>
        {flag.name}
      </h3>
      <div>
        {flag.description}
      </div>
      <a href={`/common_static/${flag.filepath}`} download>
        DOWNLOAD
      </a>
    </div>
  );
}
