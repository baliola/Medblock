module.exports = {
  presets: ['module:@react-native/babel-preset'],
  plugins: [
    'nativewind/babel',
    [
      'module-resolver',
      {
        root: ['.'],
        extensions: [
          '.ios.ts',
          '.android.ts',
          '.ts',
          '.ios.tsx',
          '.android.tsx',
          '.tsx',
          '.jsx',
          '.js',
          '.json',
        ],
        alias: {
          '@config': './src/config',
          '@models': './src/data/models',
          '@services': './src/data/services',
          '@repositories': './src/data/repositories',
          '@constants': './src/constants',
          '@components': './src/presentation/components',
          '@layouts': './src/presentation/layouts',
          '@screens': './src/presentation/modules/',
          '@utils': './src/utils/',
          '@global': './src/presentation/global/',
        },
      },
    ],
  ],
};
