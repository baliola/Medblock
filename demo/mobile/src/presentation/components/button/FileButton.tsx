import GeneralImage from '@components/image/GeneralImage';
import PDFView from 'react-native-view-pdf';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import Strings from '@constants/strings';
import {convertFileToBase64} from '@utils/convertToFile';
import {styled} from 'nativewind';
import React, {useEffect, useState} from 'react';
import {
  GestureResponderEvent,
  Image,
  TouchableOpacity,
  View,
} from 'react-native';

const StyledView = styled(View);
const StyledTouchableOpacity = styled(TouchableOpacity);
const StyledImage = styled(Image);

interface FileButtonProps {
  onPress?: ((event: GestureResponderEvent) => void) | undefined;
  file?: File | null;
  uri?: string | null;
  loading?: boolean;
}

const FileButton: React.FC<FileButtonProps> = ({
  file,
  loading,
  onPress,
  uri,
}) => {
  const [baseFile, setBaseFile] = useState<string>('');

  const handleFile = async () => {
    const base = await convertFileToBase64(file as File);
    setBaseFile(base);
  };

  useEffect(() => {
    handleFile();
  }, [file]);

  if (loading) {
    return (
      <StyledView className="items-center p-10 bg-slate-200 mb-4 rounded-2xl flex flex-col">
        <GeneralImage url={Images.logo} size={52} />
        <TextPrimary
          text={Strings.label.loading}
          classStyle="text-gray-500"
          isBold
        />
      </StyledView>
    );
  } else {
    return uri !== null ? (
      <StyledTouchableOpacity
        onPress={onPress}
        className="bg-slate-200 rounded-2xl flex flex-col justify-center items-center mt-2 mb-4 h-36 w-full">
        {uri?.endsWith('pdf') ? (
          <PDFView
            fadeInDuration={250.0}
            style={{flex: 1, borderRadius: 20}}
            resource={baseFile !== '' ? baseFile : uri}
            resourceType={baseFile !== '' ? 'base64' : 'url'}
            onError={error => {
              console.log('====================================');
              console.log('ERROR FILE --> ', error.message);
              console.log('====================================');
            }}
          />
        ) : (
          <StyledImage
            source={{uri: uri}}
            className="h-full w-full"
            style={{objectFit: 'scale-down'}}
          />
        )}
      </StyledTouchableOpacity>
    ) : (
      <StyledTouchableOpacity
        onPress={onPress}
        className="bg-slate-200 rounded-2xl flex flex-col justify-center items-center mt-2 mb-4 h-36">
        <GeneralImage url={Images.idCard} size={32} />
        <TextPrimary
          text={Strings.label.uploadYourIdCard}
          classStyle="text-gray-500 mt-2"
          isBold
        />
      </StyledTouchableOpacity>
    );
  }
};

export default FileButton;
