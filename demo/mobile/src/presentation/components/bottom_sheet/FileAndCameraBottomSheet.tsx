import React from 'react';
import BasicBottomSheet from './BasicBottomSheet';
import {styled} from 'nativewind';
import {View} from 'react-native';
import IconBigButton from '@components/button/IconBigButton';
import Images from '@constants/images';
import Strings from '@constants/strings';
import {CameraOptions, launchCamera} from 'react-native-image-picker';
import DocumentPicker from 'react-native-document-picker';
import {convertAssetToFile, convertToFile} from '@utils/convertToFile';
import Colors from '@constants/colors';
import {useSnackbar} from '@components/snackbar/SnackBar';
import {useTranslation} from 'react-i18next';

const StyledView = styled(View);

interface FileAndCameraBottomSheetProps {
  visible: boolean;
  setLoading?: (value: boolean) => void | undefined;
  onClose: ((value: boolean) => void) | undefined;
  fileType?: string | null;
  getFileFromCamera: (file?: File | null) => void;
  getFileFromLibrary: (file?: File | null) => void;
  getUri?: (uri: string) => void;
}

const FileAndCameraBottomSheet: React.FC<FileAndCameraBottomSheetProps> = ({
  onClose,
  visible,
  fileType = DocumentPicker.types.pdf,
  getFileFromCamera,
  setLoading,
  getFileFromLibrary,
  getUri,
}) => {
  const {showSnackbar} = useSnackbar();
  const {t} = useTranslation('global');

  const launchNativeCamera = () => {
    let options: CameraOptions = {
      includeBase64: true,
      quality: 0.9,
      mediaType: 'photo',
      maxHeight: 1024,
      maxWidth: 1024,
    };
    launchCamera(options, async response => {
      if (setLoading) {
        setLoading(true);
      }

      if (response.didCancel) {
        showSnackbar('User cancelled image picker', Colors.red);
      } else if (response.errorCode) {
        showSnackbar('ImagePicker Error: ' + response.errorMessage, Colors.red);
      } else {
        onClose!(false);
        const file = await convertAssetToFile(response);
        getFileFromCamera(file);
        getUri!(response.assets![0].uri as string);
      }

      if (setLoading) {
        setLoading(false);
      }
    });
  };

  const selectPDF = async () => {
    try {
      const result = await DocumentPicker.pickSingle({
        type: fileType as string,
        copyTo: 'cachesDirectory',
      });

      onClose!(false);
      if (setLoading) {
        setLoading(true);
      }
      const file = await convertToFile(result);

      if (file.size >= 2 * 1024 * 1024) {
        throw t(Strings.validation.fileMax2Mb);
      }

      getFileFromLibrary(file);
      getUri!(result.fileCopyUri as string);
      if (setLoading) {
        setLoading(false);
      }
    } catch (err) {
      if (setLoading) {
        setLoading(false);
      }
      showSnackbar(`${err}`, Colors.red);
      console.log('FilePicker Error: ', err);
      throw err;
    }
  };

  return (
    <BasicBottomSheet
      onClose={() => onClose!(false)}
      visible={visible}
      child={
        <StyledView className="flex flex-row w-full justify-around items-center h-full">
          <IconBigButton
            icon={Images.uploadFile}
            label={Strings.label.uploadFile}
            onPress={() => {
              selectPDF();
            }}
          />
          <IconBigButton
            icon={Images.camera}
            label={Strings.label.takePhoto}
            onPress={() => {
              launchNativeCamera();
            }}
          />
        </StyledView>
      }
    />
  );
};

export default FileAndCameraBottomSheet;
