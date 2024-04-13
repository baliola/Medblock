import { ReactElement } from 'react';

interface InfoItemProps {
  icon: ReactElement;
  label?: string;
  data: string;
  subData?: string;
  classStyle?: string;
}

const InfoItem: React.FC<InfoItemProps> = ({
  data,
  icon,
  label,
  classStyle,
  subData,
}) => {
  return (
    <div className={'flex flex-row items-center space-x-4 mb-4 ' + classStyle}>
      {icon}
      <div>
        {label ? <p className="text-gray-800 text-xs mb-1">{label}</p> : null}
        <p className="text-gray-800 font-bold">{data}</p>
        {subData ? (
          <p className="text-gray-800 text-xs mt-1">{subData}</p>
        ) : null}
      </div>
    </div>
  );
};

export default InfoItem;
