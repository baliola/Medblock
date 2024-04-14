interface MetaItemProps {
  label: string;
  data: string;
}

const MetaItem: React.FC<MetaItemProps> = ({ data, label }) => {
  return (
    <div className="flex flex-col">
      <p className="text-gray-800 text-xs">{label}</p>
      <p className="text-gray-800 text-lg font-bold">{data}</p>
    </div>
  );
};

export default MetaItem;
