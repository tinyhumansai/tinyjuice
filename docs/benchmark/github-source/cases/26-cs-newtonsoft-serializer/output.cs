#region License
// Copyright (c) 2007 James Newton-King
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
#endregion

using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Runtime.Serialization.Formatters;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;
using Newtonsoft.Json.Utilities;
using System.Runtime.Serialization;
using ErrorEventArgs = Newtonsoft.Json.Serialization.ErrorEventArgs;
using System.Runtime.CompilerServices;
using System.Diagnostics.CodeAnalysis;

namespace Newtonsoft.Json
{
    /// <summary>
    /// Serializes and deserializes objects into and from the JSON format.
    /// The <see cref="JsonSerializer"/> enables you to control how objects are encoded into JSON.
    /// </summary>
    public class JsonSerializer
    {
        internal TypeNameHandling _typeNameHandling;
        internal TypeNameAssemblyFormatHandling _typeNameAssemblyFormatHandling;
        internal PreserveReferencesHandling _preserveReferencesHandling;
        internal ReferenceLoopHandling _referenceLoopHandling;
        internal MissingMemberHandling _missingMemberHandling;
        internal ObjectCreationHandling _objectCreationHandling;
        internal NullValueHandling _nullValueHandling;
        internal DefaultValueHandling _defaultValueHandling;
        internal ConstructorHandling _constructorHandling;
        internal MetadataPropertyHandling _metadataPropertyHandling;
        internal JsonConverterCollection? _converters;
        internal IContractResolver _contractResolver;
        internal ITraceWriter? _traceWriter;
        internal IEqualityComparer? _equalityComparer;
        internal ISerializationBinder _serializationBinder;
        internal StreamingContext _context;
        private IReferenceResolver? _referenceResolver;

        private Formatting? _formatting;
        private DateFormatHandling? _dateFormatHandling;
        private DateTimeZoneHandling? _dateTimeZoneHandling;
        private DateParseHandling? _dateParseHandling;
        private FloatFormatHandling? _floatFormatHandling;
        private FloatParseHandling? _floatParseHandling;
        private StringEscapeHandling? _stringEscapeHandling;
        private CultureInfo _culture;
        private int? _maxDepth;
        private bool _maxDepthSet;
        private bool? _checkAdditionalContent;
        private string? _dateFormatString;
        private bool _dateFormatStringSet;

        /// <summary>
        /// Occurs when the <see cref="JsonSerializer"/> errors during serialization and deserialization.
        /// </summary>
        public virtual event EventHandler<ErrorEventArgs>? Error;

        /// <summary>
        /// Gets or sets the <see cref="IReferenceResolver"/> used by the serializer when resolving references.
        /// </summary>
        public virtual IReferenceResolver? ReferenceResolver
            { … 12 line(s) … ⟦tj:274f987f3f5e2d17aa86efb66df1517b⟧ }

        /// <summary>
        /// Gets or sets the <see cref="SerializationBinder"/> used by the serializer when resolving type names.
        /// </summary>
        [Obsolete("Binder is obsolete. Use SerializationBinder instead.")]
        public virtual SerializationBinder Binder
        {
            get
            {
                if (_serializationBinder is SerializationBinder legacySerializationBinder)
                {
                    return legacySerializationBinder;
                }

                if (_serializationBinder is SerializationBinderAdapter adapter)
                {
                    return adapter.SerializationBinder;
                }

                throw new InvalidOperationException("Cannot get SerializationBinder because an ISerializationBinder was previously set.");
            }
            set
            {
                if (value == null)
                {
                    throw new ArgumentNullException(nameof(value), "Serialization binder cannot be null.");
                }

                _serializationBinder = value as ISerializationBinder ?? new SerializationBinderAdapter(value);
            }
        }

        /// <summary>
        /// Gets or sets the <see cref="ISerializationBinder"/> used by the serializer when resolving type names.
        /// </summary>
        public virtual ISerializationBinder SerializationBinder
            { … 12 line(s) … ⟦tj:60677b24249652b5d83b32c95bddbe29⟧ }

        /// <summary>
        /// Gets or sets the <see cref="ITraceWriter"/> used by the serializer when writing trace messages.
        /// </summary>
        /// <value>The trace writer.</value>
        public virtual ITraceWriter? TraceWriter
        {
            get => _traceWriter;
            set => _traceWriter = value;
        }

        /// <summary>
        /// Gets or sets the equality comparer used by the serializer when comparing references.
        /// </summary>
        /// <value>The equality comparer.</value>
        public virtual IEqualityComparer? EqualityComparer
        {
            get => _equalityComparer;
            set => _equalityComparer = value;
        }

        /// <summary>
        /// Gets or sets how type name writing and reading is handled by the serializer.
        /// The default value is <see cref="Json.TypeNameHandling.None" />.
        /// </summary>
        /// <remarks>
        /// <see cref="JsonSerializer.TypeNameHandling"/> should be used with caution when your application deserializes JSON from an external source.
        /// Incoming types should be validated with a custom <see cref="JsonSerializer.SerializationBinder"/>
        /// when deserializing with a value other than <see cref="TypeNameHandling.None"/>.
        /// </remarks>
        public virtual TypeNameHandling TypeNameHandling
            { … 12 line(s) … ⟦tj:0260ce6b5107f2cf54ee9bbee414fccd⟧ }

        /// <summary>
        /// Gets or sets how a type name assembly is written and resolved by the serializer.
        /// The default value is <see cref="FormatterAssemblyStyle.Simple" />.
        /// </summary>
        /// <value>The type name assembly format.</value>
        [Obsolete("TypeNameAssemblyFormat is obsolete. Use TypeNameAssemblyFormatHandling instead.")]
        public virtual FormatterAssemblyStyle TypeNameAssemblyFormat
            { … 12 line(s) … ⟦tj:1b241df287916240fddb0d728b8b022c⟧ }

        /// <summary>
        /// Gets or sets how a type name assembly is written and resolved by the serializer.
        /// The default value is <see cref="Json.TypeNameAssemblyFormatHandling.Simple" />.
        /// </summary>
        /// <value>The type name assembly format.</value>
        public virtual TypeNameAssemblyFormatHandling TypeNameAssemblyFormatHandling
            { … 12 line(s) … ⟦tj:32c0a54e82cf7baf50a6d5fecd8d4189⟧ }

        /// <summary>
        /// Gets or sets how object references are preserved by the serializer.
        /// The default value is <see cref="Json.PreserveReferencesHandling.None" />.
        /// </summary>
        public virtual PreserveReferencesHandling PreserveReferencesHandling
            { … 12 line(s) … ⟦tj:2dd321d726caa0cb017a40b514ed68b9⟧ }

        /// <summary>
        /// Gets or sets how reference loops (e.g. a class referencing itself) is handled.
        /// The default value is <see cref="Json.ReferenceLoopHandling.Error" />.
        /// </summary>
        public virtual ReferenceLoopHandling ReferenceLoopHandling
            { … 12 line(s) … ⟦tj:9738ab328494aa248fc837b8a5548fed⟧ }

        /// <summary>
        /// Gets or sets how missing members (e.g. JSON contains a property that isn't a member on the object) are handled during deserialization.
        /// The default value is <see cref="Json.MissingMemberHandling.Ignore" />.
        /// </summary>
        public virtual MissingMemberHandling MissingMemberHandling
            { … 12 line(s) … ⟦tj:536986361fc76786a7a29e350d4f10e9⟧ }

        /// <summary>
        /// Gets or sets how null values are handled during serialization and deserialization.
        /// The default value is <see cref="Json.NullValueHandling.Include" />.
        /// </summary>
        public virtual NullValueHandling NullValueHandling
            { … 12 line(s) … ⟦tj:011e4bd582597250172c477ab4554f33⟧ }

        /// <summary>
        /// Gets or sets how default values are handled during serialization and deserialization.
        /// The default value is <see cref="Json.DefaultValueHandling.Include" />.
        /// </summary>
        public virtual DefaultValueHandling DefaultValueHandling
            { … 12 line(s) … ⟦tj:ee9613e5e3c23807d6255548470e5e36⟧ }

        /// <summary>
        /// Gets or sets how objects are created during deserialization.
        /// The default value is <see cref="Json.ObjectCreationHandling.Auto" />.
        /// </summary>
        /// <value>The object creation handling.</value>
        public virtual ObjectCreationHandling ObjectCreationHandling
            { … 12 line(s) … ⟦tj:831fcfdc6116e92d382106ead3e93561⟧ }

        /// <summary>
        /// Gets or sets how constructors are used during deserialization.
        /// The default value is <see cref="Json.ConstructorHandling.Default" />.
        /// </summary>
        /// <value>The constructor handling.</value>
        public virtual ConstructorHandling ConstructorHandling
            { … 12 line(s) … ⟦tj:e55e3e171077141d7bb05dee9db52ec1⟧ }

        /// <summary>
        /// Gets or sets how metadata properties are used during deserialization.
        /// The default value is <see cref="Json.MetadataPropertyHandling.Default" />.
        /// </summary>
        /// <value>The metadata properties handling.</value>
        public virtual MetadataPropertyHandling MetadataPropertyHandling
            { … 12 line(s) … ⟦tj:d3d007ee11bc5b7e212e9d073f1efe55⟧ }

        /// <summary>
        /// Gets a collection <see cref="JsonConverter"/> that will be used during serialization.
        /// </summary>
        /// <value>Collection <see cref="JsonConverter"/> that will be used during serialization.</value>
        public virtual JsonConverterCollection Converters
            { … 11 line(s) … ⟦tj:9823164f9e760d59b75c8202b910505e⟧ }

        /// <summary>
        /// Gets or sets the contract resolver used by the serializer when
        /// serializing .NET objects to JSON and vice versa.
        /// </summary>
        public virtual IContractResolver ContractResolver
        {
            get => _contractResolver;
            set => _contractResolver = value ?? DefaultContractResolver.Instance;
        }

        /// <summary>
        /// Gets or sets the <see cref="StreamingContext"/> used by the serializer when invoking serialization callback methods.
        /// </summary>
        /// <value>The context.</value>
        public virtual StreamingContext Context
        {
            get => _context;
            set => _context = value;
        }

        /// <summary>
        /// Indicates how JSON text output is formatted.
        /// The default value is <see cref="Json.Formatting.None" />.
        /// </summary>
        public virtual Formatting Formatting
        {
            get => _formatting ?? JsonSerializerSettings.DefaultFormatting;
            set => _formatting = value;
        }

        /// <summary>
        /// Gets or sets how dates are written to JSON text.
        /// The default value is <see cref="Json.DateFormatHandling.IsoDateFormat" />.
        /// </summary>
        public virtual DateFormatHandling DateFormatHandling
        {
            get => _dateFormatHandling ?? JsonSerializerSettings.DefaultDateFormatHandling;
            set => _dateFormatHandling = value;
        }

        /// <summary>
        /// Gets or sets how <see cref="DateTime"/> time zones are handled during serialization and deserialization.
        /// The default value is <see cref="Json.DateTimeZoneHandling.RoundtripKind" />.
        /// </summary>
        public virtual DateTimeZoneHandling DateTimeZoneHandling
        {
            get => _dateTimeZoneHandling ?? JsonSerializerSettings.DefaultDateTimeZoneHandling;
            set => _dateTimeZoneHandling = value;
        }

        /// <summary>
        /// Gets or sets how date formatted strings, e.g. <c>"\/Date(1198908717056)\/"</c> and <c>"2012-03-21T05:40Z"</c>, are parsed when reading JSON.
        /// The default value is <see cref="Json.DateParseHandling.DateTime" />.
        /// </summary>
        public virtual DateParseHandling DateParseHandling
        {
            get => _dateParseHandling ?? JsonSerializerSettings.DefaultDateParseHandling;
            set => _dateParseHandling = value;
        }

        /// <summary>
        /// Gets or sets how floating point numbers, e.g. 1.0 and 9.9, are parsed when reading JSON text.
        /// The default value is <see cref="Json.FloatParseHandling.Double" />.
        /// </summary>
        public virtual FloatParseHandling FloatParseHandling
        {
            get => _floatParseHandling ?? JsonSerializerSettings.DefaultFloatParseHandling;
            set => _floatParseHandling = value;
        }

        /// <summary>
        /// Gets or sets how special floating point numbers, e.g. <see cref="Double.NaN"/>,
        /// <see cref="Double.PositiveInfinity"/> and <see cref="Double.NegativeInfinity"/>,
        /// are written as JSON text.
        /// The default value is <see cref="Json.FloatFormatHandling.String" />.
        /// </summary>
        public virtual FloatFormatHandling FloatFormatHandling
        {
            get => _floatFormatHandling ?? JsonSerializerSettings.DefaultFloatFormatHandling;
            set => _floatFormatHandling = value;
        }

        /// <summary>
        /// Gets or sets how strings are escaped when writing JSON text.
        /// The default value is <see cref="Json.StringEscapeHandling.Default" />.
        /// </summary>
        public virtual StringEscapeHandling StringEscapeHandling
        {
            get => _stringEscapeHandling ?? JsonSerializerSettings.DefaultStringEscapeHandling;
            set => _stringEscapeHandling = value;
        }

        /// <summary>
        /// Gets or sets how <see cref="DateTime"/> and <see cref="DateTimeOffset"/> values are formatted when writing JSON text,
        /// and the expected date format when reading JSON text.
        /// The default value is <c>"yyyy'-'MM'-'dd'T'HH':'mm':'ss.FFFFFFFK"</c>.
        /// </summary>
        public virtual string DateFormatString
        {
            get => _dateFormatString ?? JsonSerializerSettings.DefaultDateFormatString;
            set
            {
                _dateFormatString = value;
                _dateFormatStringSet = true;
            }
        }

        /// <summary>
        /// Gets or sets the culture used when reading JSON.
        /// The default value is <see cref="CultureInfo.InvariantCulture"/>.
        /// </summary>
        public virtual CultureInfo Culture
        {
            get => _culture ?? JsonSerializerSettings.DefaultCulture;
            set => _culture = value;
        }

        /// <summary>
        /// Gets or sets the maximum depth allowed when reading JSON. Reading past this depth will throw a <see cref="JsonReaderException"/>.
        /// A null value means there is no maximum.
        /// The default value is <c>64</c>.
        /// </summary>
        public virtual int? MaxDepth
            { … 13 line(s) … ⟦tj:eaa3ad083e90c0b4024b9abc4dc9ea58⟧ }

        /// <summary>
        /// Gets a value indicating whether there will be a check for additional JSON content after deserializing an object.
        /// The default value is <c>false</c>.
        /// </summary>
        /// <value>
        /// 	<c>true</c> if there will be a check for additional JSON content after deserializing an object; otherwise, <c>false</c>.
        /// </value>
        public virtual bool CheckAdditionalContent
        {
            get => _checkAdditionalContent ?? JsonSerializerSettings.DefaultCheckAdditionalContent;
            set => _checkAdditionalContent = value;
        }

        internal bool IsCheckAdditionalContentSet()
        {
            return (_checkAdditionalContent != null);
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="JsonSerializer"/> class.
        /// </summary>
        public JsonSerializer()
            _referenceLoopHandling = JsonSerializerSettings.DefaultReferenceLoopHandling;
            _missingMemberHandling = JsonSerializerSettings.DefaultMissingMemberHandling;
            { … 11 line(s) … ⟦tj:eaf77e387528b74187f86289441f6dc0⟧ }
            _contractResolver = DefaultContractResolver.Instance;

        /// <summary>
        /// Creates a new <see cref="JsonSerializer"/> instance.
        /// The <see cref="JsonSerializer"/> will not use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/>.
        /// </summary>
        /// <returns>
        /// A new <see cref="JsonSerializer"/> instance.
        /// The <see cref="JsonSerializer"/> will not use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/>.
        /// </returns>
        public static JsonSerializer Create()
        {
            return new JsonSerializer();
        }

        /// <summary>
        /// Creates a new <see cref="JsonSerializer"/> instance using the specified <see cref="JsonSerializerSettings"/>.
        /// The <see cref="JsonSerializer"/> will not use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/>.
        /// </summary>
        /// <param name="settings">The settings to be applied to the <see cref="JsonSerializer"/>.</param>
        /// <returns>
        /// A new <see cref="JsonSerializer"/> instance using the specified <see cref="JsonSerializerSettings"/>.
        /// The <see cref="JsonSerializer"/> will not use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/>.
        /// </returns>
        public static JsonSerializer Create(JsonSerializerSettings? settings)
            { … 10 line(s) … ⟦tj:d93c3a4bcc615738babb45b13e455af5⟧ }

        /// <summary>
        /// Creates a new <see cref="JsonSerializer"/> instance.
        /// The <see cref="JsonSerializer"/> will use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/>.
        /// </summary>
        /// <returns>
        /// A new <see cref="JsonSerializer"/> instance.
        /// The <see cref="JsonSerializer"/> will use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/>.
        /// </returns>
        public static JsonSerializer CreateDefault()
        {
            // copy static to local variable to avoid concurrency issues
            JsonSerializerSettings? defaultSettings = JsonConvert.DefaultSettings?.Invoke();

            return Create(defaultSettings);
        }

        /// <summary>
        /// Creates a new <see cref="JsonSerializer"/> instance using the specified <see cref="JsonSerializerSettings"/>.
        /// The <see cref="JsonSerializer"/> will use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/> as well as the specified <see cref="JsonSerializerSettings"/>.
        /// </summary>
        /// <param name="settings">The settings to be applied to the <see cref="JsonSerializer"/>.</param>
        /// <returns>
        /// A new <see cref="JsonSerializer"/> instance using the specified <see cref="JsonSerializerSettings"/>.
        /// The <see cref="JsonSerializer"/> will use default settings 
        /// from <see cref="JsonConvert.DefaultSettings"/> as well as the specified <see cref="JsonSerializerSettings"/>.
        /// </returns>
        public static JsonSerializer CreateDefault(JsonSerializerSettings? settings)
        {
            JsonSerializer serializer = CreateDefault();
            if (settings != null)
            {
                ApplySerializerSettings(serializer, settings);
            }

            return serializer;
        }

        private static void ApplySerializerSettings(JsonSerializer serializer, JsonSerializerSettings settings)
            if (!CollectionUtils.IsNullOrEmpty(settings.Converters))
            {
            { … 127 line(s) … ⟦tj:b1838d4a4dead6a833b227d642caad95⟧ }
            }

        /// <summary>
        /// Populates the JSON values onto the target object.
        /// </summary>
        /// <param name="reader">The <see cref="TextReader"/> that contains the JSON structure to read values from.</param>
        /// <param name="target">The target object to populate values onto.</param>
        [DebuggerStepThrough]
        public void Populate(TextReader reader, object target)
        {
            Populate(new JsonTextReader(reader), target);
        }

        /// <summary>
        /// Populates the JSON values onto the target object.
        /// </summary>
        /// <param name="reader">The <see cref="JsonReader"/> that contains the JSON structure to read values from.</param>
        /// <param name="target">The target object to populate values onto.</param>
        [DebuggerStepThrough]
        public void Populate(JsonReader reader, object target)
        {
            PopulateInternal(reader, target);
        }

        internal virtual void PopulateInternal(JsonReader reader, object target)
            ValidationUtils.ArgumentNotNull(reader, nameof(reader));
            ValidationUtils.ArgumentNotNull(target, nameof(target));
            { … 22 line(s) … ⟦tj:aed2bb7cfb56c8e7d540a91642ff62a1⟧ }
            ResetReader(reader, previousCulture, previousDateTimeZoneHandling, previousDateParseHandling, previousFloatParseHandling, previousMaxDepth, previousDateFormatString);

        /// <summary>
        /// Deserializes the JSON structure contained by the specified <see cref="JsonReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="JsonReader"/> that contains the JSON structure to deserialize.</param>
        /// <returns>The <see cref="Object"/> being deserialized.</returns>
        [DebuggerStepThrough]
        public object? Deserialize(JsonReader reader)
        {
            return Deserialize(reader, null);
        }

        /// <summary>
        /// Deserializes the JSON structure contained by the specified <see cref="TextReader"/>
        /// into an instance of the specified type.
        /// </summary>
        /// <param name="reader">The <see cref="TextReader"/> containing the object.</param>
        /// <param name="objectType">The <see cref="Type"/> of object being deserialized.</param>
        /// <returns>The instance of <paramref name="objectType"/> being deserialized.</returns>
        [DebuggerStepThrough]
        public object? Deserialize(TextReader reader, Type objectType)
        {
            return Deserialize(new JsonTextReader(reader), objectType);
        }

        /// <summary>
        /// Deserializes the JSON structure contained by the specified <see cref="JsonReader"/>
        /// into an instance of the specified type.
        /// </summary>
        /// <param name="reader">The <see cref="JsonReader"/> containing the object.</param>
        /// <typeparam name="T">The type of the object to deserialize.</typeparam>
        /// <returns>The instance of <typeparamref name="T"/> being deserialized.</returns>
        [DebuggerStepThrough]
        public T? Deserialize<T>(JsonReader reader)
        {
            return (T?)Deserialize(reader, typeof(T));
        }

        /// <summary>
        /// Deserializes the JSON structure contained by the specified <see cref="JsonReader"/>
        /// into an instance of the specified type.
        /// </summary>
        /// <param name="reader">The <see cref="JsonReader"/> containing the object.</param>
        /// <param name="objectType">The <see cref="Type"/> of object being deserialized.</param>
        /// <returns>The instance of <paramref name="objectType"/> being deserialized.</returns>
        [DebuggerStepThrough]
        public object? Deserialize(JsonReader reader, Type? objectType)
        {
            return DeserializeInternal(reader, objectType);
        }

        internal virtual object? DeserializeInternal(JsonReader reader, Type? objectType)
            ValidationUtils.ArgumentNotNull(reader, nameof(reader));

            { … 23 line(s) … ⟦tj:8710bf31eafb014ac8279fb9506047e1⟧ }
            return value;

        internal void SetupReader(JsonReader reader, out CultureInfo? previousCulture, out DateTimeZoneHandling? previousDateTimeZoneHandling, out DateParseHandling? previousDateParseHandling, out FloatParseHandling? previousFloatParseHandling, out int? previousMaxDepth, out string? previousDateFormatString)
            if (_culture != null && !_culture.Equals(reader.Culture))
            {
            { … 64 line(s) … ⟦tj:3643ea124310d7e3a6a0a657182acb7f⟧ }
            }

        private void ResetReader(JsonReader reader, CultureInfo? previousCulture, DateTimeZoneHandling? previousDateTimeZoneHandling, DateParseHandling? previousDateParseHandling, FloatParseHandling? previousFloatParseHandling, int? previousMaxDepth, string? previousDateFormatString)
            // reset reader back to previous options
            if (previousCulture != null)
            { … 28 line(s) … ⟦tj:bac3388666f18fe11a11204666282758⟧ }
            }

        /// <summary>
        /// Serializes the specified <see cref="Object"/> and writes the JSON structure
        /// using the specified <see cref="TextWriter"/>.
        /// </summary>
        /// <param name="textWriter">The <see cref="TextWriter"/> used to write the JSON structure.</param>
        /// <param name="value">The <see cref="Object"/> to serialize.</param>
        public void Serialize(TextWriter textWriter, object? value)
        {
            Serialize(new JsonTextWriter(textWriter), value);
        }

        /// <summary>
        /// Serializes the specified <see cref="Object"/> and writes the JSON structure
        /// using the specified <see cref="JsonWriter"/>.
        /// </summary>
        /// <param name="jsonWriter">The <see cref="JsonWriter"/> used to write the JSON structure.</param>
        /// <param name="value">The <see cref="Object"/> to serialize.</param>
        /// <param name="objectType">
        /// The type of the value being serialized.
        /// This parameter is used when <see cref="JsonSerializer.TypeNameHandling"/> is <see cref="Json.TypeNameHandling.Auto"/> to write out the type name if the type of the value does not match.
        /// Specifying the type is optional.
        /// </param>
        public void Serialize(JsonWriter jsonWriter, object? value, Type? objectType)
        {
            SerializeInternal(jsonWriter, value, objectType);
        }

        /// <summary>
        /// Serializes the specified <see cref="Object"/> and writes the JSON structure
        /// using the specified <see cref="TextWriter"/>.
        /// </summary>
        /// <param name="textWriter">The <see cref="TextWriter"/> used to write the JSON structure.</param>
        /// <param name="value">The <see cref="Object"/> to serialize.</param>
        /// <param name="objectType">
        /// The type of the value being serialized.
        /// This parameter is used when <see cref="TypeNameHandling"/> is Auto to write out the type name if the type of the value does not match.
        /// Specifying the type is optional.
        /// </param>
        public void Serialize(TextWriter textWriter, object? value, Type objectType)
        {
            Serialize(new JsonTextWriter(textWriter), value, objectType);
        }

        /// <summary>
        /// Serializes the specified <see cref="Object"/> and writes the JSON structure
        /// using the specified <see cref="JsonWriter"/>.
        /// </summary>
        /// <param name="jsonWriter">The <see cref="JsonWriter"/> used to write the JSON structure.</param>
        /// <param name="value">The <see cref="Object"/> to serialize.</param>
        public void Serialize(JsonWriter jsonWriter, object? value)
        {
            SerializeInternal(jsonWriter, value, null);
        }

        private TraceJsonReader CreateTraceJsonReader(JsonReader reader)
        {
            TraceJsonReader traceReader = new TraceJsonReader(reader);
            if (reader.TokenType != JsonToken.None)
            {
                traceReader.WriteCurrentToken();
            }

            return traceReader;
        }

        internal virtual void SerializeInternal(JsonWriter jsonWriter, object? value, Type? objectType)
            ValidationUtils.ArgumentNotNull(jsonWriter, nameof(jsonWriter));

            { … 90 line(s) … ⟦tj:5bfdecef124fa9a00adeb497e10c0ae8⟧ }
            }

        internal IReferenceResolver GetReferenceResolver()
        {
            if (_referenceResolver == null)
            {
                _referenceResolver = new DefaultReferenceResolver();
            }

            return _referenceResolver;
        }

        internal JsonConverter? GetMatchingConverter(Type type)
        {
            return GetMatchingConverter(_converters, type);
        }

        internal static JsonConverter? GetMatchingConverter(IList<JsonConverter>? converters, Type objectType)
#if DEBUG
            ValidationUtils.ArgumentNotNull(objectType, nameof(objectType));
{ … 15 line(s) … ⟦tj:8642b7ee974f7f4fd90abd35da93fc2c⟧ }
            return null;

        internal void OnError(ErrorEventArgs e)
        {
            Error?.Invoke(this, e);
        }
    }
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (50756 bytes): call tinyjuice_retrieve with token "1aa48b9cedf9cc9dc8c491ceb9368f21"]